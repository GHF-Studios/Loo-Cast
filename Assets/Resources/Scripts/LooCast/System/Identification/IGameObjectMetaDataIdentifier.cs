namespace LooCast.System.Identification
{
    public interface IGameObjectMetaDataIdentifier : IUnityInstanceMetaDataIdentifier
    {
        #region Properties
        string GameObjectMetaDataTypeID { get; }
        string GameObjectMetaDataGUID { get; }
        string GameObjectMetaDataID { get; }
        #endregion
    }
}