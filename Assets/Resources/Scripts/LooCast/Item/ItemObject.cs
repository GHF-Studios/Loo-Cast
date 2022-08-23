using System;
using UnityEngine;

namespace LooCast.Item
{
    public abstract class ItemObject : MonoBehaviour
    {
        public Item Item
        {
            get
            {
                return item;
            }
        }
        [SerializeField] private Item item;

        private void Awake()
        {
            if (Item == null)
            {
                throw new Exception("Item is null!");
            }
        }
    }
}