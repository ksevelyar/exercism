defmodule TopSecret do
  def to_ast(string), do: Code.string_to_quoted!(string)

  defp fun_prefix([{:when, _, fun} | _]), do: fun_prefix(fun)
  defp fun_prefix([{_, _, args} | _]) when args in [[], nil], do: ""

  defp fun_prefix([{name, _, args} | _]) do
    Atom.to_string(name) |> String.slice(0..(length(args) - 1))
  end

  def decode_secret_message_part({atom, _meta, nodes} = ast, acc) when atom in [:def, :defp] do
    {ast, [fun_prefix(nodes) | acc]}
  end

  def decode_secret_message_part(ast, acc), do: {ast, acc}

  def decode_secret_message(string) do
    {_, acc} = Macro.prewalk(to_ast(string), [], &decode_secret_message_part/2)

    acc |> Enum.reverse() |> Enum.join()
  end
end
