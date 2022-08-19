defmodule Anagram do
  defp chars(word) do
    String.downcase(word) |> String.graphemes()
  end

  defp anagram?(target, candidate) when target == candidate, do: false

  defp anagram?(target, candidate) do
    Enum.sort(target) == Enum.sort(candidate)
  end

  def match(word, candidates) do
    target_chars = chars(word)
    Enum.filter(candidates, fn candidate -> anagram?(target_chars, chars(candidate)) end)
  end
end
