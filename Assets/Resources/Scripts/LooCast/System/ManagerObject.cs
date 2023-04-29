using System;

namespace LooCast.System
{
    using LooCast.System.Data;
    using LooCast.System.MetaData;
    using LooCast.System.Types;
    
    public sealed class ManagerObject : GameObjectType<ManagerObject>.GameObject
    {
        #region Properties
        public override IMetaData MetaData { get => throw new NotImplementedException(); set => throw new NotImplementedException(); }
        public override MetaData.IInstanceMetaData InstanceMetaData { get => throw new NotImplementedException(); set => throw new NotImplementedException(); }
        public override IGameObjectMetaData GameObjectMetaData { get => throw new NotImplementedException(); set => throw new NotImplementedException(); }
        public override IData Data { get => throw new NotImplementedException(); set => throw new NotImplementedException(); }
        public override IInstanceData InstanceData { get => throw new NotImplementedException(); set => throw new NotImplementedException(); }
        public override IGameObjectData GameObjectData { get => throw new NotImplementedException(); set => throw new NotImplementedException(); }
        #endregion

        #region Static Methods
#nullable enable
        public static ManagerObject CreateManagerObject()
        {
            return CreateGameObject<ManagerObject>();
        }
#nullable disable
        #endregion

        #region Overrides
        public override bool Validate()
        {
            throw new NotImplementedException();
        }
        
        protected override void PreConstruct()
        {
            base.PreConstruct();

            UnityEngine.GameObject.DontDestroyOnLoad(UnityEngineGameObject);
        }
        #endregion
    }
}
