using System;
using System.Collections.Generic;

namespace LooCast.Universe
{
    using LooCast.Core;
    using LooCast.System.Numerics;
    
    public sealed class Scale : Entity
    {
        #region Properties
        public ScaleUnityComponent ScaleUnityComponent { get; private set; }
        public int ScaleLevel { get; private set; }
        public Universe Universe { get; private set; }
        #endregion

        #region Fields
        private Dictionary<BigVec2Int, Chunk> chunkDictionary;
        #endregion

        #region Constructors
        public Scale(int scaleLevel, Universe universe) : base()
        {
            chunkDictionary = new Dictionary<BigVec2Int, Chunk>();
            
            ScaleLevel = scaleLevel;
            Universe = universe;

            EnableUnityBridge();
        }
        #endregion

        #region Methods
        public bool IsChunkGenerated(BigVec2Int chunkPosition)
        {
            return chunkDictionary.ContainsKey(chunkPosition);
        }

        public void GenerateChunk(BigVec2Int chunkPosition)
        {
            if (chunkDictionary.ContainsKey(chunkPosition))
            {
                throw new Exception($"Chunk '{chunkPosition}' has already been generated!");
            }
            
            Chunk chunk = new Chunk(chunkPosition, Universe.ChunkSize, this);
            chunkDictionary.Add(chunkPosition, chunk);
        }

        public void DeleteChunk(BigVec2Int chunkPosition)
        {
            if (!chunkDictionary.ContainsKey(chunkPosition))
            {
                throw new Exception($"Chunk '{chunkPosition}' has already been deleted!");
            }

            chunkDictionary[chunkPosition].DisableUnityBridge();
            chunkDictionary.Remove(chunkPosition);
        }

        public Chunk GetChunk(BigVec2Int chunkPosition)
        {
            if (!chunkDictionary.TryGetValue(chunkPosition, out Chunk chunk))
            {
                throw new Exception($"Chunk '{chunkPosition}' has not been generated!");
            }

            return chunk;
        }
        #endregion

        #region Overrides
        public override void EnableUnityBridge()
        {
            base.EnableUnityBridge();
            
            UnityBridge.RootGameObject.name = ToString();
            UnityBridge.RootGameObject.transform.SetParent(Universe.UnityBridge.RootGameObject.transform);

            ScaleUnityComponent = UnityBridge.RootGameObject.AddComponent<ScaleUnityComponent>();
            ScaleUnityComponent.InitializeScale(this);
        }

        public override void DisableUnityBridge()
        {
            base.DisableUnityBridge();

            ScaleUnityComponent = null;
        }
        
        public override string ToString()
        {
            return $"10^{ScaleLevel}";
        }
        #endregion
    }
}
