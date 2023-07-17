namespace LooCast.Core
{
    public abstract class Entity : IEntity
    {
        #region Properties
        public string EntityID { get; private set; }
        public IData Data { get; protected set; }
        public ILogic Logic { get; protected set; }
        public UnityBridge UnityBridge { get; private set; }
        #endregion

        #region Constructors
        protected Entity(string entityID)
        {
            EntityID = entityID;
        }
        #endregion

        #region Methods
        public virtual void EnableUnityBridge()
        {
            UnityBridge = new UnityBridge();
        }

        public virtual void DisableUnityBridge()
        {
            UnityBridge.Terminate();
            UnityBridge = null;
        }
        #endregion
    }
}
