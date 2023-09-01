defmodule MatchingBrackets do
  @doc """
  Checks that all the brackets and braces in the string are matched correctly, and nested correctly
  """
  @spec check_brackets(String.t()) :: boolean
  def check_brackets(str) do
    brackets =
      str
      |> String.graphemes()
      |> Enum.filter(&(&1 in ["{", "}", "[", "]", "(", ")"]))

    midPos = div(length(brackets), 2)

    {open_brackets, closed_brackets} = Enum.split(brackets, midPos)

    [open_brackets, Enum.reverse(closed_brackets)]
    |> Enum.zip_with(fn [o, c] -> pair?(o, c) end)
    |> Enum.all?(&(&1 == true))
  end

  defp pair?("{", "}"), do: true
  defp pair?("[", "]"), do: true
  defp pair?("(", ")"), do: true
  defp pair?(_, _), do: false
end
