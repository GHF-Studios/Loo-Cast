using System;
using System.Collections.Generic;

namespace LooCast.Universe
{
    using global::LooCast.System.Numerics;
    using global::LooCast.System.ECS;
    
    public sealed class Universe : Entity
    {
        #region Properties
        public UniverseUnityComponent UniverseUnityComponent { get; private set; }
        public int ChunkSize { get; private set; }
        #endregion

        #region Fields
        private Dictionary<int, Scale> scaleDictionary;
        #endregion

        #region Constructors
        public Universe(int chunkSize) : base()
        {
            ChunkSize = chunkSize;
            
            scaleDictionary = new Dictionary<int, Scale>();

            EnableUnityBridge();
        }
        #endregion

        #region Methods
        public bool IsScaleGenerated(int scaleLevel)
        {
            return scaleDictionary.ContainsKey(scaleLevel);
        }

        public void GenerateScale(int scaleLevel)
        {
            if (scaleDictionary.ContainsKey(scaleLevel))
            {
                throw new Exception($"Scale '{scaleLevel}' has already been generated!");
            }

            Scale scale = new Scale(scaleLevel, this);
            scaleDictionary.Add(scaleLevel, scale);
        }
        
        public Scale GetScale(int scaleLevel)
        {
            if (!scaleDictionary.TryGetValue(scaleLevel, out Scale scale))
            {
                throw new Exception($"Scale '{scaleLevel}' has not been generated!");
            }
            
            return scale;
        }

        public void DeleteScale(int scaleLevel)
        {
            if (!scaleDictionary.ContainsKey(scaleLevel))
            {
                throw new Exception($"Scale '{scaleLevel}' has already been deleted!");
            }

            scaleDictionary[scaleLevel].DisableUnityBridge();
            scaleDictionary.Remove(scaleLevel);
        }
        #endregion

        #region Overrides
        public override void EnableUnityBridge()
        {
            base.EnableUnityBridge();

            UnityBridge.RootGameObject.name = ToString();

            UniverseUnityComponent = UnityBridge.RootGameObject.AddComponent<UniverseUnityComponent>();
            UniverseUnityComponent.Setup(this);
        }

        public override void DisableUnityBridge()
        {
            base.DisableUnityBridge();

            UniverseUnityComponent = null;
        }

        public override string ToString()
        {
            return $"Universe";
        }
        #endregion
    }
}
