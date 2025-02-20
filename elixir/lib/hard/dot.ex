defmodule Dot do
  alias Dot.Graph

  defmacro graph(do: block) do
    lines =
      case block do
        {:__block__, _meta, lines} -> lines
        line -> [line]
      end

    graph =
      Enum.reduce(lines, %Graph{}, fn line, acc ->
        handle_line(line, acc)
      end)

    quote do
      unquote(Macro.escape(graph))
    end
  end

  defp map_values([values]) when is_list(values), do: Map.new(values)
  defp map_values(_), do: %{}

  defp handle_line({:--, _, [{edge1, _, _values1}, {edge2, _, values2}]}, acc) do
    edge = {edge1, edge2, map_values(values2)}

    acc = handle_line({edge1, nil, nil}, acc)
    acc = handle_line({edge2, nil, nil}, acc)

    %{acc | edges: [edge | acc.edges]}
  end

  defp handle_line({:graph, _context, values}, acc) do
    values = map_values(values)
    Map.update!(acc, :attrs, &Map.merge(&1, values))
  end

  defp handle_line({name, _context, values}, acc) do
    if is_tuple(name) || name == :--, do: raise(ArgumentError)

    values =
      case values do
        [values] when is_tuple(values) ->
          raise(ArgumentError)

        [values] ->
          Map.new(values)

        _ ->
          %{}
      end

    node = Map.merge(acc.nodes[name] || %{}, values)
    %{acc | nodes: Map.put(acc.nodes, name, node)}
  end

  defp handle_line(_line, _acc) do
    raise(ArgumentError)
  end
end
