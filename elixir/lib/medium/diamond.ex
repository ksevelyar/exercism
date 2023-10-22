defmodule Diamond do
  @doc """
  Given a letter, it prints a diamond starting with 'A',
  with the supplied letter at the widest point.
  """
  @spec build_shape(char) :: String.t()
  def build_shape(letter) do
    letters = letters(letter)
    center = Enum.count(?A..letter)

    letters
    |> Enum.zip(rows(center))
    |> Enum.map(fn {letter, row} -> render_row(cols(center), letter, row) end)
    |> Enum.join()
  end

  defp render_row(cols, letter, row) do
    chars = Enum.map(cols, fn col -> if row == col, do: letter, else: ' ' end)

    List.to_string(chars) <> "\n"
  end

  defp letters(?A), do: [?A]
  defp letters(letter), do: Enum.concat(?A..(letter - 1), letter..?A)

  defp cols(1), do: [0]
  defp cols(num), do: Enum.concat(0..(num - 2), (num - 1)..0)

  defp rows(1), do: [0]
  defp rows(num), do: Enum.concat((num - 1)..0, 1..(num - 1))
end
