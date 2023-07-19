using System;
using System.Collections.Generic;

namespace LooCast.Universe
{
    using global::System.Net.NetworkInformation;
    using LooCast.Core;
    
    public sealed class Universe : Entity
    {
        #region Properties
        public Scale CurrentScale { get; private set; }
        public int ChunkSize { get; private set; }
        public UniverseUnityComponent UniverseUnityComponent { get; private set; }
        #endregion

        #region Fields
        private Dictionary<int, Scale> scaleDictionary;
        #endregion

        #region Constructors
        public Universe() : base()
        {
            scaleDictionary = new Dictionary<int, Scale>();

            ChunkSize = 16;

            EnableUnityBridge();

            GenerateScale(0);
            CurrentScale = GetScale(0);
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
        #endregion

        #region Overrides
        public override void EnableUnityBridge()
        {
            base.EnableUnityBridge();

            UnityBridge.RootGameObject.name = ToString();

            UniverseUnityComponent = UnityBridge.RootGameObject.AddComponent<UniverseUnityComponent>();
            UniverseUnityComponent.InitializeUniverse(this);
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
