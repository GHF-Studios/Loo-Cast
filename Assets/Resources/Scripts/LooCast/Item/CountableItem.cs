using System;

namespace LooCast.Item
{
    using Data;

    public abstract class CountableItem : Item
    {
        public int MaxCount { get; protected set; }
        public int Count
        {
            get
            {
                return count;
            }

            set
            {
                if (value > MaxCount)
                {
                    throw new ArgumentOutOfRangeException("Count cannot be greater than MaxCount!");
                }
                else if (value < 0)
                {
                    throw new ArgumentOutOfRangeException("Count cannot be less than or equal to 0!");
                }
                else
                {
                    count = value;
                }
            }
        }
        private int count;

        public CountableItem(CountableItemData data, int count) : base(data)
        {
            MaxCount = data.MaxCount;
            Count = count;
        }

        public bool IsFull()
        {
            return Count >= MaxCount;
        }

        public bool CanFit(int count)
        {
            return Count + count <= MaxCount;
        }

        public int GetFreeCount()
        {
            return MaxCount - Count;
        }
    }
}