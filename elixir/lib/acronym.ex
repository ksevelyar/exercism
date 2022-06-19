defmodule Acronym do
  defp letter_from_word(word), do: String.first(word) |> String.capitalize()

  def abbreviate(string) do
    string
    |> String.split([" ", "-", "_"], trim: true)
    |> Enum.map(&letter_from_word/1)
    |> Enum.join()
  end
end
