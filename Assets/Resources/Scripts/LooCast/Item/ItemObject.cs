using System;
using UnityEngine;

namespace LooCast.Item
{
    using LooCast.Core;
    using LooCast.Inventory;

    [RequireComponent(typeof(SpriteRenderer))]
    public abstract class ItemObject : ExtendedMonoBehaviour
    {
        public Item Item { get; protected set; }
        public SpriteRenderer SpriteRenderer { get; protected set; }

        protected void Initialize(Item item)
        {
            Item = item;
            SpriteRenderer = GetComponent<SpriteRenderer>();
            SpriteRenderer.sprite = item.Sprite;
        }

        private void OnTriggerEnter2D(Collider2D collider)
        {
            if (collider.CompareTag("Player"))
            {
                PlayerInventory playerInventory = collider.GetComponent<PlayerInventory>();
                ItemContainer playerItemContainer = playerInventory.RuntimeData.ItemContainer;
                if (playerItemContainer.CanFit(Item))
                {
                    playerItemContainer.AddItem(Item);
                    Destroy(gameObject);
                }
            }
        }
    }
}