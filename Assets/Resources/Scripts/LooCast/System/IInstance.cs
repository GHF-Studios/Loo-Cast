namespace LooCast.System
{
    using System.Identification;
    
    public interface IInstance
    {
        #region Properties
        IInstanceIdentifier InstanceIdentifier { get; }
        #endregion
    }
}