defmodule PigLatin do
  @doc """
  Given a `phrase`, translate it a word at a time to Pig Latin.
  """
  @vowels ~w(a e i o u)

  @spec translate(phrase :: String.t()) :: String.t()
  def translate(phrase) do
    String.split(phrase, " ", trim: true)
    |> Enum.map(&translate_word/1)
    |> Enum.join(" ")
  end

  defp translate_word(<<ch1::bitstring-size(8), ch2::bitstring-size(8), _rest::binary>> = word) do
    if (ch1 in ~w(x y) and ch2 not in @vowels) or ch1 in @vowels do
      "#{word}ay"
    else
      "#{maybe_move_consonant_cluster(word)}ay"
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
    if head in @vowels or (acc != "" and head == "y") do
      move_consonant_cluster(<<>>, head <> rest <> acc)
    else
      move_consonant_cluster(rest, acc <> head)
    end
  end
end
