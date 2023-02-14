namespace LooCast
{
    public interface IInstanceIdentifier : IGenericIdentifier<Instance>
    {
        #region Properties
        string InstanceID { get; }
        #endregion
    }
}