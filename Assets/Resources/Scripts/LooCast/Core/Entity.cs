using System;

namespace LooCast.Core
{
    public abstract class Entity : IEntity
    {
        #region Properties
        public Guid EntityID { get; private set; }
        public IData Data { get; protected set; }
        public ILogic Logic { get; protected set; }
        public UnityBridge UnityBridge { get; private set; }
        public bool IsUnityBridgeEnabled => UnityBridge != null;
        #endregion

        #region Constructors
        protected Entity()
        {
            EntityID = Guid.NewGuid();
        }
        #endregion

        #region Methods
        public virtual void EnableUnityBridge()
        {
            if (IsUnityBridgeEnabled)
            {
                throw new Exception("UnityBridge is already enabled!");
            }
            
            UnityBridge = new UnityBridge();
            UnityBridge.RootGameObject.name = "New Entity";
        }

        public virtual void DisableUnityBridge()
        {
            if (!IsUnityBridgeEnabled)
            {
                throw new Exception("UnityBridge is already disabled!");
            }
            
            UnityBridge.Terminate();
            UnityBridge = null;
        }
        #endregion

        #region Overrides
        public override int GetHashCode()
        {
            return EntityID.GetHashCode();
        }

        public override bool Equals(object obj)
        {
            if (obj is not Entity)
            {
                return false;
            }

            Entity other = (Entity)obj;
            return other.EntityID == this.EntityID;
        }

        public override string ToString()
        {
            return EntityID.ToString();
        }
        #endregion
    }
}
