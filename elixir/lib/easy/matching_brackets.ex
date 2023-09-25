defmodule MatchingBrackets do
  @doc """
  Checks that all the brackets and braces in the string are matched correctly, and nested correctly
  """
  @spec check_brackets(String.t()) :: boolean
  def check_brackets(str) do
    str
    |> String.graphemes()
    |> Enum.filter(&(&1 in ["{", "}", "[", "]", "(", ")"]))
    |> remove_balanced_brackets()
    |> Enum.empty?()
  end

  defp remove_balanced_brackets(brackets) do
    Enum.reduce_while(brackets, [], fn
      bracket, acc when bracket in ["{", "[", "("] ->
        {:cont, [bracket | acc]}

      "}", ["{" | rest] ->
        {:cont, rest}

      ")", ["(" | rest] ->
        {:cont, rest}

      "]", ["[" | rest] ->
        {:cont, rest}

      bracket, _acc ->
        {:halt, [bracket]}
    end)
  end
end
