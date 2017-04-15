class Hamming
  def self.compute(a, b)
    raise ArgumentError if a.length != b.length

    dist = 0
    a.split('').each_index { |i| dist += 1 if a[i] != b[i] }
    dist
  end
end

module BookKeeping
  VERSION = 3
end
