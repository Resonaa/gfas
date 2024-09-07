
using namespace System.Management.Automation
using namespace System.Management.Automation.Language

Register-ArgumentCompleter -Native -CommandName 'gfas' -ScriptBlock {
    param($wordToComplete, $commandAst, $cursorPosition)

    $commandElements = $commandAst.CommandElements
    $command = @(
        'gfas'
        for ($i = 1; $i -lt $commandElements.Count; $i++) {
            $element = $commandElements[$i]
            if ($element -isnot [StringConstantExpressionAst] -or
                $element.StringConstantType -ne [StringConstantType]::BareWord -or
                $element.Value.StartsWith('-') -or
                $element.Value -eq $wordToComplete) {
                break
        }
        $element.Value
    }) -join ';'

    $completions = @(switch ($command) {
        'gfas' {
            [CompletionResult]::new('--endpoint', '--endpoint', [CompletionResultType]::ParameterName, 'GitHub API endpoint')
            [CompletionResult]::new('--no-color', '--no-color', [CompletionResultType]::ParameterName, 'Disable color printing')
            [CompletionResult]::new('-v', '-v', [CompletionResultType]::ParameterName, 'Increase logging verbosity')
            [CompletionResult]::new('--verbose', '--verbose', [CompletionResultType]::ParameterName, 'Increase logging verbosity')
            [CompletionResult]::new('-q', '-q', [CompletionResultType]::ParameterName, 'Decrease logging verbosity')
            [CompletionResult]::new('--quiet', '--quiet', [CompletionResultType]::ParameterName, 'Decrease logging verbosity')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('-V', '-V ', [CompletionResultType]::ParameterName, 'Print version')
            [CompletionResult]::new('--version', '--version', [CompletionResultType]::ParameterName, 'Print version')
            break
        }
    })

    $completions.Where{ $_.CompletionText -like "$wordToComplete*" } |
        Sort-Object -Property ListItemText
}
