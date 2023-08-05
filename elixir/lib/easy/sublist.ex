defmodule Sublist do
  @doc """
  Returns whether the first list is a sublist or a superlist of the second list
  and if not whether it is equal or unequal to the second list.
  """
  def compare(a, b) when a == b, do: :equal

  def compare(a, b) when length(a) > length(b) do
    case sublist(a, b) do
      true -> :superlist
      false -> :unequal
    end
  end

  def compare(a, b) do
    case sublist(b, a) do
      true -> :sublist
      false -> :unequal
    end
  end

  defp sublist(_a, []), do: true

  defp sublist(a, b) do
    possible_matches =
      Enum.with_index(a) |> Enum.filter(&(elem(&1, 0) == hd(b))) |> Enum.map(&elem(&1, 1))

    Enum.any?(possible_matches, &(Enum.slice(a, &1..(&1 + length(b) - 1)) == b))
  end
end
