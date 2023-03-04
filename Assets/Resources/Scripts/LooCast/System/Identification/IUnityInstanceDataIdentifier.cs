namespace LooCast.System.Identification
{
    public interface IUnityInstanceDataIdentifier : IInstanceDataIdentifier
    {
        #region Properties
        string UnityInstanceDataTypeID { get; }
        string UnityInstanceDataGUID { get; }
        string UnityInstanceDataID { get; }
        #endregion
    }
}