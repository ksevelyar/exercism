defmodule SecretHandshake do
  @actions [
    {0b1000, "jump"},
    {0b100, "close your eyes"},
    {0b10, "double blink"},
    {0b1, "wink"},
    {0b10000, "reverse"}
  ]

  @doc """
  Determine the actions of a secret handshake based on the binary
  representation of the given `code`.

  If the following bits are set, include the corresponding action in your list
  of commands, in order from lowest to highest.

  1 = wink
  10 = double blink
  100 = close your eyes
  1000 = jump

  10000 = Reverse the order of the operations in the secret handshake
  """
  @spec commands(code :: integer) :: list(String.t())
  def commands(code), do: commands(code, @actions, [])

  defguardp is_valid(code, reply_code) when Bitwise.band(code, reply_code) != 0

  defp commands(_, [], replies), do: replies

  defp commands(code, [{reply_code, reply} | actions], replies) when is_valid(code, reply_code) do
    case reply do
      "reverse" -> Enum.reverse(replies)
      _ -> commands(code, actions, [reply | replies])
    end
  end

  defp commands(code, [_ | actions], replies), do: commands(code, actions, replies)
end
