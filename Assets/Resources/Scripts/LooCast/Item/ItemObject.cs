using System;
using UnityEngine;

namespace LooCast.Item
{
    using LooCast.Core;
    using LooCast.Util;

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
    }
}