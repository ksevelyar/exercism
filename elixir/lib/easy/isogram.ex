defmodule Isogram do
  @doc """
  Determines if a word or sentence is an isogram
  """
  @spec isogram?(String.t()) :: boolean
  def isogram?(sentence) do
    sentence
    |> String.downcase()
    |> String.split("", trim: true)
    |> Stream.filter(&(&1 not in ["-", " "]))
    |> Enum.frequencies()
    |> Enum.all?(fn {_letter, frequency} -> frequency < 2 end)
  end
end
