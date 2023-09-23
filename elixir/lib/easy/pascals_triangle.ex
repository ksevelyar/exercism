defmodule PascalsTriangle do
  @doc """
  Calculates the rows of a pascal triangle
  with the given height
  """
  @spec rows(integer) :: [[integer]]
  def rows(num) do
    do_rows(num, 1, [])
  end

  defp do_rows(num, count, rows) when count > num do
    Enum.reverse(rows)
  end

  defp do_rows(num, count, []) do
    do_rows(num, count + 1, [row(count, nil)])
  end

  defp do_rows(num, count, [prev | _rest] = rows) do
    do_rows(num, count + 1, [row(count, prev) | rows])
  end

  defp row(1, _prev), do: [1]
  defp row(2, _prev), do: [1, 1]

  defp row(n, prev) do
    Enum.map(1..n, fn
      1 ->
        1

      ^n ->
        1

      n ->
        Enum.at(prev, n - 2) + Enum.at(prev, n - 1)
    end)
  end
end
