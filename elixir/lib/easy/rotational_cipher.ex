defmodule RotationalCipher do
  @alphabet_length Enum.count(?a..?z)

  @doc """
  Given a plaintext and amount to shift by, return a rotated string.

  Example:
  iex> RotationalCipher.rotate("Attack at dawn", 13)
  "Nggnpx ng qnja"
  """
  @spec rotate(text :: String.t(), shift :: integer) :: String.t()
  def rotate(text, shift) do
    text = String.to_charlist(text)

    rotate_each_char(text, shift, [])
  end

  defp rotate_each_char([], _shift, acc) do
    acc
    |> Enum.reverse()
    |> List.to_string()
  end

  defp rotate_each_char([char | tail], shift, acc) do
    new_char = rotate_char(char, shift)
    rotate_each_char(tail, shift, [new_char | acc])
  end

  defp rotate_char(char, shift) when char in ?A..?Z, do: shift_char(?A, char, shift)
  defp rotate_char(char, shift) when char in ?a..?z, do: shift_char(?a, char, shift)
  defp rotate_char(char, _shift), do: char

  defp shift_char(start, char, shift) do
    start + rem(char - start + shift, @alphabet_length)
  end
end
