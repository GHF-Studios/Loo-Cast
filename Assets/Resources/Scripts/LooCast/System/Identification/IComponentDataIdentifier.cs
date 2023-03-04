namespace LooCast.System.Identification
{
    public interface IComponentDataIdentifier : IGameObjectDataIdentifier
    {
        #region Properties
        string ComponentDataTypeID { get; }
        string ComponentDataGUID { get; }
        string ComponentDataID { get; }
        #endregion
    }
}