namespace LooCast.System.Identification
{
    public interface IUnityInstanceMetaDataIdentifier : IInstanceMetaDataIdentifier
    {
        #region Properties
        string UnityInstanceMetaDataTypeID { get; }
        string UnityInstanceMetaDataGUID { get; }
        string UnityInstanceMetaDataID { get; }
        #endregion
    }
}