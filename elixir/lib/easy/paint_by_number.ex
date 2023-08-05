defmodule PaintByNumber do
  def palette_bit_size(color_count) do
    find_enough_bytes(0, color_count)
  end

  defp find_enough_bytes(byte_count, color_count) do
    case Integer.pow(2, byte_count) >= color_count do
      true -> byte_count
      false -> find_enough_bytes(byte_count + 1, color_count)
    end
  end

  def empty_picture(), do: ""

  def test_picture() do
    <<0::2, 1::2, 2::2, 3::2>>
  end

  def prepend_pixel(picture, color_count, pixel_color_index) do
    palette_size = palette_bit_size(color_count)

    <<pixel_color_index::size(palette_size), picture::bitstring>>
  end

  def get_first_pixel("", _color_count), do: nil

  def get_first_pixel(picture, color_count) do
    palette_size = palette_bit_size(color_count)
    <<pixel::size(palette_size), _rest::bitstring>> = picture

    pixel
  end

  def drop_first_pixel("", _color_count), do: ""

  def drop_first_pixel(picture, color_count) do
    palette_size = palette_bit_size(color_count)
    <<_pixel::size(palette_size), rest::bitstring>> = picture

    rest
  end

  def concat_pictures(picture1, picture2) do
    <<picture1::bitstring, picture2::bitstring>>
  end
end
