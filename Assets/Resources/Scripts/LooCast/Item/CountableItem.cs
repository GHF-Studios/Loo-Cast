using System;
using UnityEngine.Events;

namespace LooCast.Item
{
    using Data;

    public abstract class CountableItem : Item
    {
        public UnityEvent OnCountChanged
        {
            get
            {
                return onCountChanged;
            }
        }
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
                    onCountChanged.Invoke();
                }
            }
        }
        public float Density { get; protected set; }
        public float MinObjectScale { get; protected set; }
        public float MaxObjectScale { get; protected set; }

        private UnityEvent onCountChanged;
        private int count;

        public CountableItem(CountableItemData data) : base(data)
        {
            onCountChanged = new UnityEvent();

            MaxCount = data.MaxCount.Value;
            Count = data.DefaultCount.Value;
            Density = data.Density.Value;
            MinObjectScale = data.MinObjectScale.Value;
            MaxObjectScale = data.MaxObjectScale.Value;
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