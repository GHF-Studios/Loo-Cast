using System;
using System.Numerics;
using System.Collections.Generic;
using UnityEngine;

namespace LooCast.Universe
{
    using LooCast.System.Numerics;
    using LooCast.Core;
    
    public sealed class UniverseObserver : Entity
    {
        #region Properties
        public UniverseObserverUnityComponent UniverseObserverUnityComponent { get; private set; }
        public Scale CurrentScale { get; private set; }
        public int ObservingDistance { get; private set; }
        #endregion

        #region Fields
        private List<BigVec2Int> oldProximalChunkCoordinates;
        #endregion

        #region Constructors
        public UniverseObserver(int observingDistance)
        {
            Universe universe = LooCastCoreManager.Instance.Universe;
            if (!universe.IsScaleGenerated(0))
            {
                universe.GenerateScale(0);
            }
            
            CurrentScale = universe.GetScale(0);

            if (observingDistance % universe.ChunkSize != 0)
            {
                throw new ArgumentException("Observing distance must be a multiple of the chunk size!");
            }

            ObservingDistance = observingDistance;

            oldProximalChunkCoordinates = new List<BigVec2Int>();

            EnableUnityBridge();
        }
        #endregion

        #region Callbacks
        public void Update()
        {
            Universe universe = LooCastCoreManager.Instance.Universe;

            List<BigVec2Int> proximalChunkCoordinates = GetProximalChunkCoordinates();

            foreach (BigVec2Int proximalChunkCoordinate in proximalChunkCoordinates)
            {
                if (!CurrentScale.IsChunkGenerated(proximalChunkCoordinate))
                {
                    CurrentScale.GenerateChunk(proximalChunkCoordinate);
                }
            }

            foreach (BigVec2Int oldProximalChunkCoordinate in oldProximalChunkCoordinates)
            {
                if (!proximalChunkCoordinates.Contains(oldProximalChunkCoordinate))
                {
                    if (CurrentScale.IsChunkGenerated(oldProximalChunkCoordinate))
                    {
                        CurrentScale.DeleteChunk(oldProximalChunkCoordinate);
                    }
                }
            }

            oldProximalChunkCoordinates = proximalChunkCoordinates;
        }
        #endregion

        #region Methods
        public List<BigVec2Int> GetProximalChunkCoordinates()
        {
            Universe universe = LooCastCoreManager.Instance.Universe;
            int chunkSize = universe.ChunkSize;
            List<BigVec2Int> potentialChunkCoordinates = new List<BigVec2Int>();
            BigVec2Int currentPosition = (BigVec2Int)UniverseObserverUnityComponent.transform.position;
            BigInteger minX = currentPosition.X - ObservingDistance;
            BigInteger maxX = currentPosition.X + ObservingDistance;
            BigInteger minY = currentPosition.Y - ObservingDistance;
            BigInteger maxY = currentPosition.Y + ObservingDistance;

            for (BigInteger x = minX; x < maxX; x += chunkSize)
            {
                for (BigInteger y = minY; y < maxY; y += chunkSize)
                {
                    potentialChunkCoordinates.Add(new BigVec2Int(x, y) / chunkSize);
                }
            }

            return potentialChunkCoordinates;
        }
        #endregion

        #region Overrides
        public override void EnableUnityBridge()
        {
            base.EnableUnityBridge();

            UnityBridge.RootGameObject.name = "Universe Observer";

            UniverseObserverUnityComponent = UnityBridge.RootGameObject.AddComponent<UniverseObserverUnityComponent>();
            UniverseObserverUnityComponent.InitializeUniverseObserver(this);
        }

        public override void DisableUnityBridge()
        {
            base.DisableUnityBridge();

            UniverseObserverUnityComponent = null;
        }
        #endregion
    }
}
