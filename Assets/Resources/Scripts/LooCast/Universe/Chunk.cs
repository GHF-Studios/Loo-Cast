using System;

namespace LooCast.Universe
{
    using global::LooCast.System.ECS;
    using global::LooCast.System.Numerics;
    
    public sealed class Chunk : Entity
    {
        #region Properties
        public ChunkUnityComponent ChunkUnityComponent { get; private set; }
        public BigVec2Int ChunkPosition { get; private set; }
        public BigVec2Int Position { get; private set; }
        public int Size { get; private set; }
        public Scale Scale { get; private set; }
        #endregion

        #region Constructors
        public Chunk(BigVec2Int chunkPosition, int size, Scale scale) : base()
        {
            ChunkPosition = chunkPosition;
            Position = chunkPosition * size;
            Size = size;
            Scale = scale;

            EnableUnityBridge();
        }
        #endregion

        #region Overrides
        public override void EnableUnityBridge()
        {
            base.EnableUnityBridge();

            UnityBridge.RootGameObject.name = ToString();
            UnityBridge.RootGameObject.transform.SetParent(Scale.UnityBridge.RootGameObject.transform);

            ChunkUnityComponent = UnityBridge.RootGameObject.AddComponent<ChunkUnityComponent>();
            ChunkUnityComponent.Setup(this);
        }

        public override void DisableUnityBridge()
        {
            base.DisableUnityBridge();

            ChunkUnityComponent = null;
        }

        public override string ToString()
        {
            return $"{Scale}_{ChunkPosition}";
        }
        #endregion
    }
}
