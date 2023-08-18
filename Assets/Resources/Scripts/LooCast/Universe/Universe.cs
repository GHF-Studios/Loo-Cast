using System;
using System.Collections.Generic;

namespace LooCast.Universe
{
    using LooCast.System.ECS;
    using LooCast.System.Lua;

    [LuaNamespace("LooCast.Universe")]
    public sealed class Universe : Entity
    {
        #region Properties
        public UniverseUnityComponent UniverseUnityComponent { get; private set; }
        #endregion

        #region Fields
        public int chunkSize;
        private Dictionary<int, Scale> scaleDictionary;
        #endregion

        #region Constructors
        public Universe(int chunkSize = 32) : base()
        {
            this.chunkSize = chunkSize;
            
            scaleDictionary = new Dictionary<int, Scale>();

            EnableUnityBridge();
        }
        #endregion

        #region Static Methods
        #endregion

        #region Methods
        [LuaMethod("GetChunkSize")]
        public int GetChunkSize()
        {
            return chunkSize;
        }

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
