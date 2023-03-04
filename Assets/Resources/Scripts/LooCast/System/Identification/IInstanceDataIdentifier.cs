namespace LooCast.System.Identification
{
    public interface IInstanceDataIdentifier : IDataIdentifier
    {
        #region Properties
        string InstanceDataTypeID { get; }
        string InstanceDataGUID { get; }
        string InstanceDataID { get; }
        #endregion
    }
}