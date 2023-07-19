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

        #region Unity Callbacks
        private void Start()
        {
            transform.localPosition = (Vector3)Chunk.Position;
        }

        private void OnDrawGizmos()
        {
            if (Chunk is not null)
            {
                Gizmos.color = Color.green;
                Gizmos.DrawWireCube(transform.position + (Vector3.one * (Chunk.Size / 2)), Vector3.one * Chunk.Size);
            }
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
