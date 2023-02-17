namespace LooCast.System.Identification
{
    public interface IGameObjectIdentifier : IUnityInstanceIdentifier
    {
        #region Properties
        string GameObjectTypeID { get; }
        string GameObjectGUID { get; }
        string GameObjectID { get; }
        #endregion  
    }
}