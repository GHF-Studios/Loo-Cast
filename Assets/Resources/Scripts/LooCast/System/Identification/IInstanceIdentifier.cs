namespace LooCast.System.Identification
{
    public interface IInstanceIdentifier : IIdentifier, IInstanceTypeIdentifier
    {
        #region Properties
        string InstanceTypeID { get; }
        string InstanceGUID { get; }
        string InstanceID { get; }
        #endregion
    }
}