namespace LooCast.System.Identification
{
    public interface IGameObjectDataIdentifier : IUnityInstanceDataIdentifier
    {
        #region Properties
        string GameObjectDataTypeID { get; }
        string GameObjectDataGUID { get; }
        string GameObjectDataID { get; }
        #endregion
    }
}