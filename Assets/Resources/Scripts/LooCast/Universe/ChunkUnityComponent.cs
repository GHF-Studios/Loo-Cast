using System;
using UnityEngine;

namespace LooCast.Universe
{
    using LooCast.Core;
    
    public sealed class ChunkUnityComponent : UnityComponent
    {
        #region Properties
        public Chunk Chunk { get; private set; }
        #endregion

        #region Fields
        private GameObject debugObject;
        private SpriteRenderer debugSpriteRenderer;
        #endregion

        #region Unity Callbacks
        private void Awake()
        {
            
        }

        private void Start()
        {
            debugObject = new GameObject();
            debugSpriteRenderer = debugObject.AddComponent<SpriteRenderer>();
            debugSpriteRenderer.sprite = Resources.Load<Sprite>("Textures/UI/default");
            debugSpriteRenderer.color = new Color(1, 1, 1, 0);
            debugObject.transform.SetParent(transform);
            debugObject.transform.localScale = Vector3.one * Chunk.Size * 0.95f;

            transform.localPosition = (Vector3)Chunk.Position;
            transform.localScale = Vector3.one;
        }
        
        private void FixedUpdate()
        {
            if (Chunk is not null)
            {
                debugSpriteRenderer.color = new Color(1, 1, 1, Chunk.Scale.AlphaFactor);
            }
        }

        private void OnDrawGizmos()
        {
            // if (Chunk is not null)
            // {
            //     Gizmos.color = Color.Lerp(new Color(1, 0, 0, 0.5f), new Color(0, 1, 0, 1), Chunk.Scale.ScaleFactor);
            //     Gizmos.DrawWireCube(
            //         (transform.position + (Vector3.one * (Chunk.Size / 2))) * Chunk.Scale.ScaleFactor,
            //         Vector3.one * Chunk.Size * Chunk.Scale.ScaleFactor
            //         );
            // }
        }
        #endregion

        #region Methods
        public void InitializeChunk(Chunk chunk)
        {
            if (Chunk is not null)
            {
                throw new InvalidOperationException($"Chunk reference has already been initialized!");
            }

            Chunk = chunk;
        }
        #endregion
    }
}
