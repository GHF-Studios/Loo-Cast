namespace LooCast.System.Identification
{
    public interface IComponentMetaDataIdentifier : IGameObjectMetaDataIdentifier
    {
        #region Properties
        string ComponentMetaDataTypeID { get; }
        string ComponentMetaDataGUID { get; }
        string ComponentMetaDataID { get; }
        #endregion
    }
}