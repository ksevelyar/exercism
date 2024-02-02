defmodule VariableLengthQuantity do
  import Bitwise

  @vlq_data_length 7
  @u32_chunks 4

  @doc """
  Encode integers into a bitstring of VLQ encoded bytes
  """
  @spec encode(integers :: [integer]) :: binary
  def encode(integers) do
    integers
    |> Enum.flat_map(&vlq_from_u32/1)
    |> List.foldr(<<>>, fn item, acc -> <<item>> <> acc end)
  end

  defp vlq_from_u32(num) do
    Enum.map(0..@u32_chunks, fn ind ->
      vlq_byte = num >>> (@vlq_data_length * ind) &&& 0b1111111

      {ind, vlq_byte}
    end)
    |> List.foldr([], fn {ind, item}, acc ->
      case ind do
        0 -> [item | acc]
        _ when item == 0 and acc == [] -> acc
        _ -> [item ||| 0b10000000 | acc]
      end
    end)
    |> Enum.reverse()
  end

  @doc """
  Decode a bitstring of VLQ encoded bytes into a series of integers
  """
  @spec decode(bytes :: binary) :: {:ok, [integer]} | {:error, String.t()}
  def decode(bytes) do
    bytes = for <<byte::8 <- bytes>>, do: byte

    {buffer, vlq_numbers} = slice_bytes_to_numbers(bytes)

    case buffer do
      [] -> {:ok, Enum.map(vlq_numbers, &vlq_to_u32/1) |> Enum.reverse()}
      _ -> {:error, "incomplete sequence"}
    end
  end

  defp slice_bytes_to_numbers(bytes) do
    Enum.reduce(bytes, {[], []}, fn item, {cur, nums} ->
      case item >>> @vlq_data_length do
        0 -> {[], [[item | cur] | nums]}
        _ -> {[item | cur], nums}
      end
    end)
  end

  defp vlq_to_u32(bytes) do
    bytes
    |> Stream.with_index()
    |> Enum.reduce(0, fn {byte, ind}, acc ->
      shift = (byte &&& 0b1111111) <<< (@vlq_data_length * ind)

      acc ||| shift
    end)
  end
end
