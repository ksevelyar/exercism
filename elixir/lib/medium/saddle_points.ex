defmodule SaddlePoints do
  @doc """
  Parses a string representation of a matrix
  to a list of rows
  """
  @spec rows(String.t()) :: [[integer]]
  def rows(""), do: []

  def rows(str) do
    str
    |> String.split(~r/\R/)
    |> Enum.map(fn row ->
      row |> String.split(" ") |> Enum.map(&String.to_integer/1)
    end)
  end

  @doc """
  Parses a string representation of a matrix
  to a list of columns
  """
  @spec columns(String.t()) :: [[integer]]
  def columns(""), do: []

  def columns(str) do
    rows = rows(str)
    col_indexes = List.first(rows) |> Enum.with_index()

    Enum.map(col_indexes, fn {_, ind} ->
      Enum.map(rows, &Enum.at(&1, ind))
    end)
  end

  @doc """
  Calculates all the saddle points from a string
  representation of a matrix
  """
  @spec saddle_points(String.t()) :: [{integer, integer}]
  def saddle_points(str) do
    rows = rows(str)
    cols = columns(str)

    rows
    |> Enum.with_index()
    |> Enum.flat_map(fn {row, row_ind} ->
      max_in_row = Enum.max(row)

      row
      |> Stream.with_index()
      |> Stream.filter(fn {value, col_ind} ->
        min_in_col = Enum.min(Enum.at(cols, col_ind))

        value == max_in_row && value == min_in_col
      end)
      |> Enum.map(fn {_val, col_ind} -> {row_ind + 1, col_ind + 1} end)
    end)
  end
end
