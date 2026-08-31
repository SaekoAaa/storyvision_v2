{{- define "storyvision-mysql.name" -}}
{{- default .Chart.Name .Values.nameOverride | trunc 63 | trimSuffix "-" -}}
{{- end -}}

{{- define "storyvision-mysql.labels" -}}
app.kubernetes.io/name: {{ include "storyvision-mysql.name" . }}
app.kubernetes.io/instance: {{ .Release.Name }}
app.kubernetes.io/managed-by: {{ .Release.Service }}
app.kubernetes.io/component: database
{{- end -}}
