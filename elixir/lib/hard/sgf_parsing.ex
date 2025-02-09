defmodule SgfParsing do
  defmodule Sgf do
    defstruct properties: %{}, children: []
  end

  @type sgf :: %Sgf{properties: map, children: [sgf]}

  @doc """
  Parse a string into a Smart Game Format tree
  """
  @spec parse(encoded :: String.t()) :: {:ok, sgf} | {:error, String.t()}
  def parse(encoded) do
    with {:ok, node} <- validate_tree_presence(encoded) do
      parse_node(node)
    end
  end

  defp parse_node(node) do
    {properties, children} =
      case Regex.split(~r/(?<=\])(?=[;(])/, node, parts: 2) do
        [properties, children] -> {properties, children}
        [properties] -> {properties, ""}
      end

    with {:ok, properties} <- parse_properties(properties),
         {:ok, children} <- parse_childs(children) do
      {:ok, %SgfParsing.Sgf{properties: properties, children: children}}
    end
  end

  defp validate_tree_presence(string) do
    case Regex.scan(~r/\((.*)\)/s, string) do
      [[_, ""]] -> {:error, "tree with no nodes"}
      [[_, tree]] -> {:ok, tree}
      [] -> {:error, "tree missing"}
    end
  end

  defp parse_properties(";"), do: {:ok, %{}}

  defp parse_properties(properties) do
    properties = Regex.split(~r/(?<=\])(?=[A-Z])/, properties)

    result =
      Enum.reduce_while(properties, %{}, fn prop, acc ->
        case parse_property(prop) do
          {:ok, {key, prop}} -> {:cont, Map.put(acc, key, prop)}
          error -> {:halt, error}
        end
      end)

    case result do
      {:error, _} = error -> error
      result -> {:ok, result}
    end
  end

  defp parse_property(property) do
    regex = ~r/(;?[A-Za-z]+|\[(?:\\\]|[^\]])*\])/

    [[_, key] | values] = Regex.scan(regex, property)

    cond do
      values == [] ->
        {:error, "properties without delimiter"}

      String.upcase(key) != key ->
        {:error, "property must be in uppercase"}

      true ->
        key = String.trim_leading(key, ";")
        values = Enum.map(values, fn [_, value] -> extract_value(value) end)
        {:ok, {key, values}}
    end
  end

  defp extract_value(value) do
    value
    |> String.trim_leading("[")
    |> String.replace(~r/\]$/, "")
    |> String.replace(~r/\\\t/, " ")
    |> String.replace(~r/\\\n/, "")
    |> String.replace(~r/\\n/, "n")
    |> String.replace(~r/\\t/, "t")
    |> String.replace(~r/[^\S\r\n]/, "\s")
    |> String.replace("\\]", "]")
    |> String.replace("\\\\", "\\")
  end

  defp parse_childs(str) do
    children =
      Regex.scan(~r/\(;([^()]+)\)/, str)
      |> Enum.map(fn [_, inner] -> inner end)

    children =
      case children do
        [] ->
          case String.split(str, ";", trim: true) do
            [children] -> [children]
            _ -> []
          end

        children ->
          children
      end

    result =
      Enum.reduce_while(children, [], fn node, acc ->
        case parse_node(node) do
          {:ok, node} -> {:cont, [node | acc]}
          error -> {:halt, error}
        end
      end)

    case result do
      {:error, _} = error ->
        error

      _ ->
        {:ok, Enum.reverse(result)}
    end
  end
end
