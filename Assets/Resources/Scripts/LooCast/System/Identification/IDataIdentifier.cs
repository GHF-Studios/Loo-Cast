namespace LooCast.System.Identification
{
    public interface IDataIdentifier : IObjectIdentifier
    {
        #region Properties
        string DataTypeID { get; }
        string DataGUID { get; }
        string DataID { get; }
        #endregion
    }
}