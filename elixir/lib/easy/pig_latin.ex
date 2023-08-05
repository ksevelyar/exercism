defmodule PigLatin do
  @doc """
  Given a `phrase`, translate it a word at a time to Pig Latin.
  """
  @vowels ~w(a e i o u)
  @vowels_at_start ~w(xr yt)

  @spec translate(phrase :: String.t()) :: String.t()
  def translate(phrase) do
    if String.starts_with?(phrase, @vowels ++ @vowels_at_start) do
      "#{phrase}ay"
    else
      "#{maybe_move_consonant_cluster(phrase)}ay"
    end
  end

  def maybe_move_consonant_cluster(word) do
    move_consonant_cluster(word, "")
  end

  defp move_consonant_cluster(<<>>, acc) do
    acc
  end

  defp move_consonant_cluster(<<"qu", rest::binary>>, acc) do
    move_consonant_cluster(rest, acc <> "qu")
  end

  defp move_consonant_cluster(<<head::bitstring-size(8), rest::binary>>, acc) do
    if head in @vowels do
      move_consonant_cluster(<<>>, head <> rest <> acc)
    else
      move_consonant_cluster(rest, acc <> head)
    end
  end
end
