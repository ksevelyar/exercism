defmodule PigLatin do
  @doc """
  Given a `phrase`, translate it a word at a time to Pig Latin.
  """
  @vowels ~w(a e i o u)

  @spec translate(phrase :: String.t()) :: String.t()
  def translate(phrase) do
    maybe_add_yay(phrase)
  end

  defp maybe_add_yay(word) do
    if String.starts_with?(word, @vowels ++ ["xr", "yt"]) do
      "#{word}ay"
    else
      word
    end
  end
end
