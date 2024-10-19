defmodule CircularBuffer do
  @moduledoc """
  An API to a stateful process that fills and empties a circular buffer
  """

  use GenServer

  defstruct [:capacity, data: %{}, read_index: 0, write_index: 0]

  @doc """
  Create a new buffer of a given capacity
  """
  @spec new(capacity :: integer) :: {:ok, pid}
  def new(capacity) do
    GenServer.start_link(__MODULE__, capacity)
  end

  @doc """
  Read the oldest entry in the buffer, fail if it is empty
  """
  @spec read(buffer :: pid) :: {:ok, any} | {:error, atom}
  def read(buffer) do
    GenServer.call(buffer, :read)
  end

  @doc """
  Write a new item in the buffer, fail if is full
  """
  @spec write(buffer :: pid, item :: any) :: :ok | {:error, atom}
  def write(buffer, item) do
    GenServer.call(buffer, {:write, item})
  end

  @doc """
  Write an item in the buffer, overwrite the oldest entry if it is full
  """
  @spec overwrite(buffer :: pid, item :: any) :: :ok
  def overwrite(buffer, item) do
    GenServer.call(buffer, {:overwrite, item})
  end

  @doc """
  Clear the buffer
  """
  @spec clear(buffer :: pid) :: :ok
  def clear(buffer) do
    GenServer.cast(buffer, :clear)
  end

  @impl true
  def init(capacity) do
    {:ok, %CircularBuffer{capacity: capacity}}
  end

  @impl true
  def handle_call({:write, item}, _from, state) do
    if writable?(state) do
      data = Map.put(state.data, state.write_index, item)

      {:reply, :ok, %{state | data: data, write_index: state.write_index + 1}}
    else
      {:reply, {:error, :full}, state}
    end
  end

  @impl true
  def handle_call({:overwrite, item}, _from, state) do
    writable? = writable?(state)
    data = Map.put(state.data, state.write_index, item)
    state = %{state | data: data, write_index: state.write_index + 1}

    state =
      if writable? do
        state
      else
        data = Map.delete(state.data, state.read_index)
        %{state | data: data, read_index: state.read_index + 1}
      end

    {:reply, :ok, state}
  end

  @impl true
  def handle_call(:read, _from, state) do
    {item, data} = Map.pop(state.data, state.read_index, :empty)

    case item do
      :empty ->
        {:reply, {:error, :empty}, state}

      item ->
        {:reply, {:ok, item}, %{state | data: data, read_index: state.read_index + 1}}
    end
  end

  def handle_cast(:clear, state) do
    {:noreply, %{state | read_index: 0, write_index: 0, data: %{}}}
  end

  defp writable?(state) do
    state.write_index - state.read_index < state.capacity
  end
end
