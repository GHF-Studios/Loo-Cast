﻿using System;
using System.Numerics;
using System.Collections.Generic;
using UnityEngine;

namespace LooCast.Universe
{
    using LooCast.System.Numerics;
    using LooCast.System.ECS;
    using LooCast.System.Lua;
    using LooCast.Core;

    [LuaNamespace("Universe")]
    public sealed class UniverseObserver : Entity
    {
        #region Enums
        public enum ScaleTransitionType
        {
            ZoomIn,
            ZoomOut
        }
        #endregion
        
        #region Properties
        public UniverseObserverUnityComponent UniverseObserverUnityComponent { get; private set; }
        public Scale CurrentScale { get; private set; }
        public bool IsTransitioningScale { get; private set; }
        public float ScaleTransitionDuration { get; private set; }
        #endregion

        #region Fields
        private int observingDistance;
        private List<BigVec2Int> currentScaleOldProximalChunkCoordinates;
        private List<BigVec2Int> newScaleOldProximalChunkCoordinates;
        private ScaleTransitionType? scaleTransitionType;
        private Scale newCurrentScale;
        private Universe universe;
        #endregion

        #region Constructors
        public UniverseObserver(int observingDistance = 256)
        {
            universe = UniverseManager.GetUniverse();
            if (!universe.IsScaleGenerated(0))
            {
                universe.GenerateScale(0);
            }
            
            CurrentScale = universe.GetScale(0);
            CurrentScale.ScaleFactor = 1.0f;
            CurrentScale.AlphaFactor = 1.0f;

            if (observingDistance % universe.GetChunkSize() != 0)
            {
                throw new ArgumentException("Observing distance must be a multiple of the chunk size!");
            }

            this.observingDistance = observingDistance;

            currentScaleOldProximalChunkCoordinates = new List<BigVec2Int>();
            newScaleOldProximalChunkCoordinates = new List<BigVec2Int>();

            ScaleTransitionDuration = 1.5f;

            EnableUnityBridge();
        }
        #endregion

        #region Callbacks
        public void Update()
        {
            List<BigVec2Int> proximalChunkCoordinates = GetProximalChunkCoordinates();

            foreach (BigVec2Int proximalChunkCoordinate in proximalChunkCoordinates)
            {
                if (!CurrentScale.IsChunkGenerated(proximalChunkCoordinate))
                {
                    CurrentScale.GenerateChunk(proximalChunkCoordinate);
                }
                
                if (newCurrentScale is not null)
                {
                    if (!newCurrentScale.IsChunkGenerated(proximalChunkCoordinate))
                    {
                        newCurrentScale.GenerateChunk(proximalChunkCoordinate);
                    }
                }
            }
            
            foreach (BigVec2Int currentScaleOldProximalChunkCoordinate in currentScaleOldProximalChunkCoordinates)
            {
                if (!proximalChunkCoordinates.Contains(currentScaleOldProximalChunkCoordinate))
                {
                    if (CurrentScale.IsChunkGenerated(currentScaleOldProximalChunkCoordinate))
                    {
                        CurrentScale.DeleteChunk(currentScaleOldProximalChunkCoordinate);
                    }
                }
            }
            
            currentScaleOldProximalChunkCoordinates = proximalChunkCoordinates;

            if (newCurrentScale is not null)
            {
                foreach (BigVec2Int newScaleOldProximalChunkCoordinate in newScaleOldProximalChunkCoordinates)
                {
                    if (!proximalChunkCoordinates.Contains(newScaleOldProximalChunkCoordinate))
                    {
                        if (newCurrentScale.IsChunkGenerated(newScaleOldProximalChunkCoordinate))
                        {
                            newCurrentScale.DeleteChunk(newScaleOldProximalChunkCoordinate);
                        }
                    }
                }

                newScaleOldProximalChunkCoordinates = proximalChunkCoordinates;
            }
        }
        #endregion

        #region Methods
        [LuaMethod("GetObservingDistance")]
        public int GetObservingDistance()
        {
            return observingDistance;
        }
        
        public void InitializeScaleTransition(ScaleTransitionType scaleTransitionType)
        {
            if (!IsTransitioningScale)
            {
                IsTransitioningScale = true;

                if (scaleTransitionType == ScaleTransitionType.ZoomIn)
                {
                    Debug.Log("[UniverseObserver] Zooming in!");
                }
                else
                {
                    Debug.Log("[UniverseObserver] Zooming out!");
                }
                
                this.scaleTransitionType = scaleTransitionType;
                int newScaleLevel = CurrentScale.ScaleLevel;
                
                if (scaleTransitionType == ScaleTransitionType.ZoomOut)
                {
                    newScaleLevel++;
                }
                else
                {
                    newScaleLevel--;
                }
                
                if (!universe.IsScaleGenerated(newScaleLevel))
                {
                    universe.GenerateScale(newScaleLevel);
                }

                newCurrentScale = universe.GetScale(newScaleLevel);
            }
        }

        public void UpdateScaleTransition(float oldScaleFactor, float oldAlphaFactor, float newScaleFactor, float newAlphaFactor)
        {
            if (IsTransitioningScale)
            {
                CurrentScale.ScaleFactor = oldScaleFactor;
                CurrentScale.AlphaFactor = oldAlphaFactor;
                newCurrentScale.ScaleFactor = newScaleFactor;
                newCurrentScale.AlphaFactor = newAlphaFactor;
            }
        }

        public void FinalizeScaleTransition()
        {
            if (IsTransitioningScale)
            {
                if (scaleTransitionType == ScaleTransitionType.ZoomIn)
                {
                    Debug.Log("[UniverseObserver] Finished zooming in!");
                }
                else
                {
                    Debug.Log("[UniverseObserver] Finished zooming out!");
                }
                
                UpdateScaleTransition(0.0f, 0.0f, 1.0f, 1.0f);

                universe.DeleteScale(CurrentScale.ScaleLevel);
                CurrentScale = newCurrentScale;
                newCurrentScale = null;
                scaleTransitionType = null;
                IsTransitioningScale = false;
            }
        }

        private List<BigVec2Int> GetProximalChunkCoordinates()
        {
            int chunkSize = universe.GetChunkSize();
            List<BigVec2Int> potentialChunkCoordinates = new List<BigVec2Int>();
            BigVec2Int currentPosition = (BigVec2Int)UniverseObserverUnityComponent.transform.position;
            BigInteger minX = currentPosition.X - observingDistance;
            BigInteger maxX = currentPosition.X + observingDistance;
            BigInteger minY = currentPosition.Y - observingDistance;
            BigInteger maxY = currentPosition.Y + observingDistance;

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
            UniverseObserverUnityComponent.Setup(this);
        }

        public override void DisableUnityBridge()
        {
            base.DisableUnityBridge();

            UniverseObserverUnityComponent = null;
        }
        #endregion
    }
}
