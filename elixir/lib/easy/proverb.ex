defmodule Proverb do
  @doc """
  Generate a proverb from a list of strings.
  """
  @spec recite(strings :: [String.t()]) :: String.t()
  def recite([]), do: ""

  def recite([word | _rest] = words) do
    do_recite(words, [], word) |> Enum.reverse() |> Enum.join("\n")
  end

  defp do_recite([_word], acc, start) do
    ["And all for the want of a #{start}.\n" | acc]
  end

  defp do_recite([wa, wb | rest], acc, start) do
    do_recite([wb | rest], ["For want of a #{wa} the #{wb} was lost." | acc], start)
  end
end
