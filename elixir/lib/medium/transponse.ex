defmodule Transpose do
  def transpose(""), do: ""

  def transpose(input) do
    rows = input |> String.split("\n") |> Enum.map(&String.graphemes/1)
    padded_rows = rows |> Enum.zip(column_lengths(rows)) |> Enum.map(&pad_row/1)
    longest_row_length = rows |> Enum.map(&length/1) |> Enum.max()

    0..(longest_row_length - 1)
    |> Stream.map(fn row ->
      padded_rows |> Enum.map(fn padded_row -> Enum.at(padded_row, row) end) |> Enum.join()
    end)
    |> Enum.join("\n")
  end

  defp column_lengths(rows) do
    rows
    |> Enum.reverse()
    |> Stream.map(&length/1)
    |> Enum.reduce([], fn column, columns ->
      prev_column = Enum.at(columns, 0) || 0

      [max(column, prev_column) | columns]
    end)
  end

  defp pad_row({row, column_length}) do
    row ++ List.duplicate(" ", column_length - length(row))
  end
end
