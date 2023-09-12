defmodule House do
  @doc """
  Return verses of the nursery rhyme 'This is the House that Jack Built'.
  """
  @spec recite(start :: integer, stop :: integer) :: String.t()
  def recite(start, stop) do
    start..stop |> Enum.map(&verse/1) |> Enum.join()
  end

  defp verse(1), do: "This is the house that Jack built.\n"
  defp verse(2 = n), do: verse("malt", n)
  defp verse(3 = n), do: verse("rat", n)
  defp verse(4 = n), do: verse("cat", n)
  defp verse(5 = n), do: verse("dog", n)
  defp verse(6 = n), do: verse("cow with the crumpled horn", n)
  defp verse(7 = n), do: verse("maiden all forlorn", n)
  defp verse(8 = n), do: verse("man all tattered and torn", n)
  defp verse(9 = n), do: verse("priest all shaven and shorn", n)
  defp verse(10 = n), do: verse("rooster that crowed in the morn", n)
  defp verse(11 = n), do: verse("farmer sowing his corn", n)
  defp verse(12 = n), do: verse("horse and the hound and the horn", n)

  defp verse(start, n) do
    previous_verses = (n - 1)..1 |> Enum.map(&prev_verse/1) |> Enum.join(" ")

    "This is the #{start} #{previous_verses}"
  end

  defp prev_verse(1), do: "that lay in the house that Jack built.\n"
  defp prev_verse(2), do: "that ate the malt"
  defp prev_verse(3), do: "that killed the rat"
  defp prev_verse(4), do: "that worried the cat"
  defp prev_verse(5), do: "that tossed the dog"
  defp prev_verse(6), do: "that milked the cow with the crumpled horn"
  defp prev_verse(7), do: "that kissed the maiden all forlorn"
  defp prev_verse(8), do: "that married the man all tattered and torn"
  defp prev_verse(9), do: "that woke the priest all shaven and shorn"
  defp prev_verse(10), do: "that kept the rooster that crowed in the morn"
  defp prev_verse(11), do: "that belonged to the farmer sowing his corn"
end
