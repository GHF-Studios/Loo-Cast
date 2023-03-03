namespace LooCast.System.Identification
{
    public interface IDataObjectIdentifier : IDataIdentifier
    {
        #region Properties
        string DataObjectTypeID { get; }
        string DataObjectGUID { get; }
        string DataObjectID { get; }
        #endregion
    }
}