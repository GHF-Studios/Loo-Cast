namespace LooCast.Identifier
{
    public interface IIdentifiableInstance : IIdentifiable
    {
        #region Properties
        IIdentifiableType InstanceType { get; }
        long InstanceID { get; }
        #endregion
    }
}