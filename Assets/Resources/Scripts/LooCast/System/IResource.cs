namespace LooCast.System
{
    using System.Identification;
    
    public interface IResource : IInstance
    {
        #region Properties
        IResourceIdentifier ResourceIdentifier { get; }
        #endregion
    }
}